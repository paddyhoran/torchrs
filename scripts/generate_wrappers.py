import os
import sys
from string import Template, ascii_lowercase
#from ..cwrap import cwrap
#from ..cwrap.plugins import StandaloneExtension, GenericNN, NullableArguments, AutoGPU

#BASE_PATH = os.path.realpath(os.path.join(__file__, '..', '..', '..'))
BASE_PATH = os.environ['TORCH_PATH']
WRAPPER_PATH = os.path.join(BASE_PATH, 'torch', 'csrc', 'nn')
THNN_UTILS_PATH = os.path.join(BASE_PATH, 'torch', '_thnn', 'utils.py')


def import_module(name, path):
    if sys.version_info >= (3, 5):
        import importlib.util
        spec = importlib.util.spec_from_file_location(name, path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module
    elif sys.version_info >= (3, 0):
        from importlib.machinery import SourceFileLoader
        return SourceFileLoader(name, path).load_module()
    else:
        import imp
        return imp.load_source(name, path)

thnn_utils = import_module('torch._thnn.utils', THNN_UTILS_PATH)

FUNCTION_TEMPLATE = Template("""\
[[
  name: $name
  return: void
  cname: $cname
  arguments:
""")

COMMON_TRANSFORMS = {
    'THIndex_t': 'i64',
    'THCIndex_t': 'usize',
    'THInteger_t': 'i32',
    'int': 'i32'
}
COMMON_CPU_TRANSFORMS = {
    'THNNState*': 'void*',
    'THIndexTensor*': 'THLongTensor*',
    'THIntegerTensor*': 'THIntTensor*',
}
COMMON_GPU_TRANSFORMS = {
    'THCState*': 'void*',
    'THCIndexTensor*': 'THCudaLongTensor*',
}

TYPE_TRANSFORMS = {
    'Trait': {
        'int': 'i32',
        'long': 'i64',
        'THTensor*': '&mut TensorKind',
        'real': 'f32',
        'accreal': 'f64',
        'double': 'f64',
        'THIndexTensor*': '&mut TensorKind',
        'THIntegerTensor*': '&mut TensorKind',
        'THGenerator*': '&mut THGenerator'
    },
    'Float': {
        'THTensor*': 'THFloatTensor*',
        'real': 'float',
        'accreal': 'double',
    },
    'Double': {
        'THTensor*': 'THDoubleTensor*',
        'real': 'double',
        'accreal': 'double',
    },
    'CudaHalf': {
        'THCTensor*': 'THCudaHalfTensor*',
        'real': 'half',
        'accreal': 'float',
    },
    'Cuda': {
        'THCTensor*': 'THCudaTensor*',
        'real': 'float',
        'accreal': 'float',
    },
    'CudaDouble': {
        'THCTensor*': 'THCudaDoubleTensor*',
        'real': 'double',
        'accreal': 'double',
    },
}

def should_wrap_function(name):
	if name.startswith('LookupTable_'):
		return False
	return (name.endswith('updateOutput') or
		name.endswith('updateGradInput') or
		name.endswith('accGradParameters') or
		name.endswith('backward'))

for t, transforms in TYPE_TRANSFORMS.items():
    transforms.update(COMMON_TRANSFORMS)

for t in ['Float', 'Double']:
    TYPE_TRANSFORMS[t].update(COMMON_CPU_TRANSFORMS)
for t in ['CudaHalf', 'Cuda', 'CudaDouble']:
    TYPE_TRANSFORMS[t].update(COMMON_GPU_TRANSFORMS)

def rstype(arg):
	return TYPE_TRANSFORMS['Trait'].get(arg.type, arg.type)

def wrap_function_decl(name, arguments):
    cname = name
    type = 'Trait'
    declaration = ''
    declaration += '\t' + 'fn ' + cname + \
        '(&mut self, ' + ', '.join(arg.name + ': ' + TYPE_TRANSFORMS[type].get(arg.type, arg.type) for arg in arguments[1:]) + ')'
    return declaration

def arg_cast(name, argtype, type):
	usename = name
	if "Tensor" in argtype:
		usename += '.inner() as *mut {}'.format(TYPE_TRANSFORMS[type][argtype][:-1])
	return usename

def wrap_function_impl(type, name, arguments):
    cname = 'THNN_' + type + name
    impl = '\t\tunsafe {\n'
    impl += '\t\t\t' + cname + \
        '(self.state, ' + ', '.join(arg_cast(arg.name, arg.type, type) for arg in arguments[1:]) + ');\n'

    impl += '\t\t}\n'
    return impl

def generate_wrappers():
    wrap_backend_decl()
    wrap_backend_impls()
    generate_function_classes()
#    wrap_cunn()
#    wrap_generic()



def wrap_backend_decl():
	wrapper = "// Autogenerated - do not change\n"
	wrapper += "#![allow(non_snake_case)]\n\n"
	wrapper += "use rutorch::*;"
	wrapper += "use tensor::{Tensor, TensorKind};\n\n"
    #wrapper = '#include <TH/TH.h>\n\n\n'
	wrapper += 'pub trait BackendIntf {\n\n'
	wrapper += "\tfn get_state(&self) ->  *mut ::std::os::raw::c_void;\n"
	nn_functions = thnn_utils.parse_header(thnn_utils.THNN_H_PATH)
	nn_functions = filter(lambda fn: "unfolded" not in fn.name, nn_functions)
	nn_functions = filter(lambda fn: should_wrap_function(fn.name), nn_functions)

	for fn in nn_functions:
		wrapper += wrap_function_decl(fn.name, fn.arguments) + ";\n"
	wrapper += "\n}"
	with open('src/nn/backends/backend.rs', 'w') as f:
		f.write(wrapper)

self_dict = { 'is_optional': False, 'name': '&mut self', 'type': 'self' }

def wrap_backend_impl_type(type):
	wrapper = "// Autogenerated - do not change\n"
	wrapper += "#![allow(non_snake_case)]\n"
	wrapper += "#![allow(non_camel_case)]\n\n"
	wrapper += "use tensor::{Tensor, TensorKind};\n"
	wrapper += "use nn::backends::backend::*;\n"
	wrapper += "use rutorch::*;\n\n"
	wrapper += "pub struct THNN_{}Backend ".format(type) + "{\n"
	wrapper += "\tstate: *mut ::std::os::raw::c_void,\n"
	wrapper += "}\n\n"
	wrapper += "impl BackendIntf for THNN_{}Backend ".format(type) + " {\n"
	wrapper += "\tfn get_state(&self) ->  *mut ::std::os::raw::c_void {\n"
	wrapper += "\t\tself.state"
	wrapper += "\t}"
	nn_functions = thnn_utils.parse_header(thnn_utils.THNN_H_PATH)
	nn_functions = filter(lambda fn: "unfolded" not in fn.name, nn_functions)
	nn_functions = filter(lambda fn: should_wrap_function(fn.name), nn_functions)

	for fn in nn_functions:
		wrapper += wrap_function_decl(fn.name, fn.arguments) + " {\n"
		wrapper += wrap_function_impl(type, fn.name, fn.arguments)
		wrapper += "\t}\n"

	wrapper += "}\n"
	with open('src/nn/backends/thnn_{}.rs'.format(type.lower()), 'w') as f:
		f.write(wrapper)

def wrap_backend_impls():
	for t in ['Float', 'Double']:
		wrap_backend_impl_type(t)

def build_header():
	header = "// Autogenerated - do not change\n"
	header += "#![allow(non_snake_case)]\n"
	header += "#![allow(non_camel_case)]\n\n"
	header += "use autograd::{Function, FuncIntf, FuncDelegate, FIWrap};\n"
	header += "use tensor::{OptTensorKindList, TensorKindList, NewSelf, make_vec};\n"
	header += "use itertools::repeat_call;\n"
	header += "use nn::backends::backend::*;\n\n\n"
	return header

def build_forward(name, args):
	forward = "let backend = input_list[0].backend();\n"
	forward += "self.save_for_backward("
	return forward

def build_backward(name, args):
	backward = ""
	return backward


def build_args(name, args):
	fn_class = "#[builder(pattern=\"owned\")]\n"
	fn_class += "#[derive(Builder, Clone, Default)]\n"
	fn_class += "pub struct {}Args ".format(name) + "{\n"
	for arg in args:
		fn_class += "\tpub {}: {},\n".format(arg.name, rstype(arg))
	fn_class += "}\n"
	return fn_class

def _make_function_class_criterion(class_name, update_output, update_grad_input, acc_grad_parameters):
	args = [arg for arg in update_output.arguments[4:] if "Tensor" not in arg.type]
	full_args = update_output.arguments[4:]
	tensor_idxs = [idx for idx, arg in enumerate(full_args) if "Tensor" in arg.type]
	input_args = ["self.args.{}".format(arg.name) for arg in update_output.arguments[4:] if "Tensor" not in arg.type]
	output_args = ["self.args.{}".format(arg.name) for arg in update_output.arguments[4:] if "Tensor" not in arg.type]
	for i, idx in enumerate(tensor_idxs):
		output_args.insert(idx, "&mut input_list[{}].clone()".format(i+2))
		input_args.insert(idx, "&mut saved[{}].clone()".format(i+2))

	def build_forward_class_criterion():
		forward = "\t\tlet mut backend = input_list[0].backend();\n"
		forward += "\t\tself.save_for_backward(input_list);\n"
		forward += "\t\tlet mut output = input_list[0].new(1);\n"
		forward += "\t\tbackend.{}(&mut input_list[0].clone(), &mut input_list[1].clone(), &mut output, ".format(update_output.name)
		forward +=  ', '.join(arg for arg in output_args) + ");\n"
		forward += "\t\tvec![output]"
		return forward

	def build_backward_class_criterion():
		backward = "\t\tlet mut saved = self.saved_tensors();\n"
		backward += "\t\tlet mut input = saved[0].clone();\n"
		backward += "\t\tlet mut target = saved[1].clone();\n"
		backward += "\t\tlet mut grad_output = grad_output_list.remove(0).unwrap();\n"
		backward += "\t\tlet mut backend = input.backend();\n"
		backward += "\t\tlet mut grad_input = grad_output.new(()).resize_as(&input).zero_();\n"
		backward += "\t\tbackend.{}(&mut input, &mut target, &mut grad_input, ".format(update_grad_input.name)
		backward += ', '.join(arg for arg in input_args) + ");\n"
		backward += "\t\tlet mut grad_output_expanded = grad_output.view(make_vec(1, grad_input.dim() as usize).as_slice());\n"
		backward += "\t\tlet mut grad_output_expanded = grad_output_expanded.expand_as(&grad_input);\n"		
		backward += "\t\tlet grad_input = grad_input.mult_(&grad_output_expanded);\n"
		backward += "\t\tvec![Some(grad_input), None]"
		return backward

	fn_class = build_args(class_name, args)
	needs_args = len(args) >  0
	if needs_args:
		fn_class += "impl_func_args!({}, {}Args);\n".format(class_name, class_name)
	else:
		fn_class += "impl_func!({});\n".format(class_name)


	fn_class += "impl FuncIntf for {} ".format(class_name) + " {\n"
	fn_class += "\tfn forward(&mut self, input_list: &mut TensorKindList) -> TensorKindList {\n"
	fn_class += build_forward_class_criterion()
	fn_class += "\n\t}\n"
	fn_class += "\tfn backward(&mut self, grad_output_list: &mut OptTensorKindList) -> OptTensorKindList {\n"
	fn_class += build_backward_class_criterion()
	fn_class += "\n\t}\n"
	fn_class += "}\n\n"
	return fn_class



def _make_function_class(class_name, update_output, update_grad_input, acc_grad_parameters):
	return ""

def generate_function_classes():
	auto = build_header()

	nn_functions = thnn_utils.parse_header(thnn_utils.THNN_H_PATH)
	print(len(nn_functions))
	function_list = list(filter(lambda fn: "unfolded" not in fn.name, nn_functions))
	function_by_name = {fn.name: fn for fn in function_list}
	print(len(function_by_name))
	classes_to_generate = {fn.name.partition('_')[0] for fn in function_list}
	print(len(classes_to_generate))
	exceptions = {
		'Linear',
		'IndexLinear',
		'SpatialFullConvolution',
		'SpatialConvolutionMM',
		'SparseLinear',
		'TemporalConvolution',
		'SpatialAveragePooling',
		'SpatialMaxPooling',
		'SpatialDilatedMaxPooling',
		'SpatialMaxUnpooling',
		'SpatialAdaptiveMaxPooling',
		'SpatialAdaptiveAveragePooling',
		'VolumetricAveragePooling',
		'VolumetricMaxPooling',
		'VolumetricMaxUnpooling',
		'VolumetricConvolution',
		'VolumetricFullConvolution',
		'VolumetricConvolutionMM',
		'TemporalMaxPooling',
		'BatchNormalization',
		'LookupTable',
		'PReLU',
		'RReLU',
		'Threshold',
		'LeakyReLU',
		'GRUFused',
		'LSTMFused',
		'unfolded',
	}
	name_remap = {
        'TemporalConvolution': 'Conv1d',
        'SpatialDilatedConvolution': 'DilatedConv2d',
        'SpatialMaxUnpooling': 'MaxUnpool2d',
        'SpatialReflectionPadding': 'ReflectionPad2d',
        'SpatialReplicationPadding': 'ReplicationPad2d',
        'VolumetricReplicationPadding': 'ReplicationPad3d',
        'VolumetricMaxUnpooling': 'MaxUnpool3d',
        'SoftMax': 'Softmax',
        'LogSoftMax': 'LogSoftmax',
        'HardTanh': 'Hardtanh',
        'HardShrink': 'Hardshrink',
        'SoftPlus': 'Softplus',
        'SoftShrink': 'Softshrink',
        'MSECriterion': 'MSELoss',
        'AbsCriterion': 'L1Loss',
        'BCECriterion': '_BCELoss',  # TODO: move the glue code into THNN
        'ClassNLLCriterion': 'NLLLoss',
        'DistKLDivCriterion': 'KLDivLoss',
        'SpatialClassNLLCriterion': 'NLLLoss2d',
        'MultiLabelMarginCriterion': 'MultiLabelMarginLoss',
        'MultiMarginCriterion': 'MultiMarginLoss',
        'SmoothL1Criterion': 'SmoothL1Loss',
        'SoftMarginCriterion': 'SoftMarginLoss',
    }

	classes_to_generate -= exceptions
	print(*classes_to_generate)
	for fn in classes_to_generate:
		update_output = function_by_name[fn + '_updateOutput']
		update_grad_input = function_by_name[fn + '_updateGradInput']
		acc_grad_parameters = function_by_name.get(fn + '_accGradParameters')
		class_name = name_remap.get(fn, fn)
        # This has to call a function to retain correct references to functions
		if 'Criterion' in fn:
			auto += _make_function_class_criterion(class_name, update_output,
                                                 update_grad_input, acc_grad_parameters)
		else:
			auto += _make_function_class(class_name, update_output,
                                       update_grad_input, acc_grad_parameters)
	with open('src/nn/_functions/thnn/auto.rs', 'w') as f:
		f.write(auto)

def wrap_function(name, type, arguments):
    cname = 'THNN_' + type + name
    declaration = ''
    declaration += cname + \
        '(' + ', '.join(TYPE_TRANSFORMS[type].get(arg.type, arg.type) for arg in arguments) + ');\n'
    declaration += FUNCTION_TEMPLATE.substitute(name=type + name, cname=cname)
    indent = ' ' * 4
    dict_indent = ' ' * 6
    prefix = indent + '- '
    for arg in arguments:
        if not arg.is_optional:
            declaration += prefix + TYPE_TRANSFORMS[type].get(arg.type, arg.type) + ' ' + arg.name + '\n'
        else:
            t = TYPE_TRANSFORMS[type].get(arg.type, arg.type)
            declaration += prefix + 'type: ' + t + '\n' + \
                dict_indent + 'name: ' + arg.name + '\n' + \
                dict_indent + 'nullable: True' + '\n'
    declaration += ']]\n\n\n'
    return declaration

def wrap_nn():
    #wrapper = '#include <TH/TH.h>\n\n\n'
    wrapper = ''
    nn_functions = thnn_utils.parse_header(thnn_utils.THNN_H_PATH)
    for fn in nn_functions:
    	wrapper += wrap_function_trait(fn.name, fn.arguments)
    for fn in nn_functions:
        for t in ['Float', 'Double']:
            wrapper += wrap_function(fn.name, t, fn.arguments)
    with open('work/THNN.cwrap', 'w') as f:
        f.write(wrapper)
#    cwrap('torch/csrc/nn/THNN.cwrap', plugins=[
#        StandaloneExtension('torch._thnn._THNN'),
#        NullableArguments(),
#    ])


def wrap_cunn():
    wrapper = '#include <TH/TH.h>\n'
    wrapper += '#include <THC/THC.h>\n\n\n'
    cunn_functions = thnn_utils.parse_header(thnn_utils.THCUNN_H_PATH)
    for fn in cunn_functions:
        for t in ['CudaHalf', 'Cuda', 'CudaDouble']:
            wrapper += wrap_function(fn.name, t, fn.arguments)
    with open('torch/csrc/nn/THCUNN.cwrap', 'w') as f:
        f.write(wrapper)
    cwrap('torch/csrc/nn/THCUNN.cwrap', plugins=[
        StandaloneExtension('torch._thnn._THCUNN'),
        NullableArguments(),
        AutoGPU(has_self=False),
    ])

GENERIC_FUNCTION_TEMPLATE = Template("""\
[[
  name: $name
  return: void
  options:
""")

def wrap_generic_function(name, backends):
    declaration = ''
    declaration += GENERIC_FUNCTION_TEMPLATE.substitute(name=name)
    for backend in backends:
        declaration += '    - cname: ' + name + '\n'
        declaration += '      backend: ' + backend['name'] + '\n'
        declaration += '      arguments:\n'
        for arg in backend['arguments']:
            declaration += '       - arg: ' + arg.type + ' ' + arg.name + '\n'
            if arg.is_optional:
                declaration += '         optional: True\n'
    declaration += ']]\n\n\n'
    return declaration


def wrap_generic():
    from collections import OrderedDict
    defs = OrderedDict()

    def should_wrap_function(name):
        if name.startswith('LookupTable_'):
            return False
        return (name.endswith('updateOutput') or
                name.endswith('updateGradInput') or
                name.endswith('accGradParameters') or
                name.endswith('backward'))

    def add_functions(name, functions):
        for fn in functions:
            if not should_wrap_function(fn.name):
                continue
            if fn.name not in defs:
                defs[fn.name] = []
            defs[fn.name] += [{
                'name': name,
                'arguments': fn.arguments[1:],
            }]

    add_functions('nn', thnn_utils.parse_header(thnn_utils.THNN_H_PATH))
    add_functions('cunn', thnn_utils.parse_header(thnn_utils.THCUNN_H_PATH))

    wrapper = ''
    for name, backends in defs.items():
        wrapper += wrap_generic_function(name, backends)
#    with open('target/work/THNN_generic.cwrap', 'w') as f:
#        f.write(wrapper)

#    cwrap('torch/csrc/nn/THNN_generic.cwrap', plugins=[
#        GenericNN(header=True),
#    ], default_plugins=False, destination='torch/csrc/nn/THNN_generic.h')

#    cwrap('torch/csrc/nn/THNN_generic.cwrap', plugins=[
#        GenericNN(),
#    ], default_plugins=False)


if __name__ == '__main__':
	generate_wrappers()
