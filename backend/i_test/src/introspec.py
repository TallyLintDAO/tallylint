def add(a, b):
    return a + b


def function_info(func, *args):
    function_name = func.__name__
    output = func(*args)
    output_type = type(output)
    input_types = tuple(type(arg) for arg in args)

    print(f"Function Name: {function_name}")
    print(f"Input Types: {input_types}")
    print(f"Output Type: {output_type}")


# Call the function_info with add function and its parameters
function_info(add, 1, 2)
