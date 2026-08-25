# Danfoss IDL Parser

## Motivation

To create a compiled code package (CCP) that can be imported and run on Danfoss processors one must write a `.idl` (Interface Description Language) file. As far as I can tell, this is a completely bespoke format that Danfoss created. The programmer (that's you) is then supposed to process this `.idl` file with a utility that Danfoss provides called "RatatoskT3" (possibly in reference to the stupid squirrel from Norse mythology who runs up and down the world tree, but in this case he runs up and down an abstract syntax tree). This utility produces `.c` and `.h` files that correspond to the interface described (roll credits) in the `.idl` file.

This is all well and good, but I don't really like coding in C. The only "officially supported" languages are C and C++, so a lesser technomancer would be out of luck. But I am no lesser technomancer.

The programmer would fill in the implementations in the generated `.c` file, compile the files into a `.obj` file, archive that into a `.lib` file, and then zip the final `.lib` and original `.idl` files into a `.zip` file. This `.zip` file can then be imported as a CCP into Plus+1 GUIDE. *Finally*, the programmer can use the `Create Externally Defined Class` and `Call Method of Externally Defined Class` blocks within GUIDE to instantiate and call methods of the class defined in the `.idl` file and implemented in the `.c` file.

Did you catch that? *GUIDE* doesn't need a `.c` file, the *compiler* needs a `.c` file to turn into a `.obj` file (which GUIDE needs).

You know *what else* can produce `.obj` files? Rust.

The only problem is that the ABI of the `.obj` file *must match* that which would have been produced officially using C. I theorize that GUIDE uses the `.idl` itself to generate its own "map" of the `.obj` file, whcih it then uses to actaully call the methods. Officially, RatatoskT3 generates a ton of weird C preprocessor macros that are never used within the `.c` and `.h` files, but I theorize that GUIDE uses them somehow.

Thus, `danfoss-idl-parser` was born. Take in a `.idl` file, get matching Rust code, compile (targeting `thumbv7m-none-eabi` for the MC050-110), archive or rename it to a `.lib`, zip that baby up, and you're Rustin' on the Danfoss.

## Supported IDL Features

- XClass
    - Properties
    - Variables
        - Initial Values
        - Properties
    - Methods
        - Arguments
            - Initial Values
            - Properties
        - Verbatim Elements

## Unsupported IDL Features

- XClass Verbatim Elements
- XClass Method Variables
- Insert Elements