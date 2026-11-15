## Inspiration
Only **18-20%** of people speak English. **<5%** speak it as a native language (Guo, 2018).   
Still, over **95%** of all mainstream programming languages require English syntax and keywords, and tend to follow English naming conventions (Guo, 2018).   
A study was done relating to non-native English-speaking programmers in 2018.   
Because of having to constantly translate English keywords and understand English error messages, 
a massive **cognitive load penalty** was created. This forced them to learn the semantics of the English language while also considered computer logic (Guo, 2018).  
## What it does

It takes in a source file (.tpy is recommended) and transpiles it to raw Python.   
It works on several languages. It is **not** just a find-and-replace tool.   
It preserves grammatical word orders according to the language.   
It also has a powerful **alias** feature, which makes it extensible and allows it to transpile into readable English Python code, so the product is still clean, understandable code      
Here's an example script   
```
#!mr
# ^ that comment tells the transpiler what language is being used
mulya{value} = 0
#^ 'mulya' will be called 'value' (alias system), making it readable

shreni(1, 11) madhye pratyek i:
    mulya += i

@en"""
    1 te 10 paryantchya sankhyanchi berij karte

"""
#^ This comment will become english

mulya chhapa
```

Currently TuffPython supports:   
- French (fr)
- Spanish (es)
- Hindi (hi)
- Marathi (mr)
- Mandarin (zh)

## How we built it
- Programmed in Rust for blazingly fast execution speed and portability.   
- It uses the **reqwest** and **serde_json** Rust crates (libraries) to use a Google Translate API.   
- The **lexer** first takes all the input text and converts it into tokens (like strings, numbers, symbols, etc.)   
- Then, based on the chosen language, different **processor** modules are used to interpret the tokens.  
 The processors translate the code into actual Python code   
- The processors rely on the **alias** and **translation** modules to use the special features. 
- When transpilation is done, the Python runtime runs the code (TuffPython takes control of stdin, stdout, stderr)
- Whenever an error is thrown, TuffPython translates the error message and description into the developer's language, while not changing file paths.
- A [vscode extension](https://marketplace.visualstudio.com/items?itemName=tuffpython.tuffpython-language-support) is available, while adds autocomplete suggestions, syntax highlighting, and compile/run buttons to make TuffPython as easy to code with as regular Python is.   
## Challenges we ran into
- I started with trying to program everything myself manually. By the time I finished coding the lexer and parts of the first processor, I realized I would not be able to finish the project in time like this. For the first time, I've used AI to help assist me with my project. Instead of AI generating the whole thing, I gave it specific tasks based on the architecture, rules, exceptions, and requirements.  
- While writing example scripts, I realized that people wouldn't use this with the transpiler alone. So, I decided to create a vscode extension to make it nice to work with.  
## Accomplishments that we're proud of
- Making the translation features
- Designing the special syntax and alias system
- Publishing a helpful vscode extension
- Working with AI agent for the first time  

## What we learned
- How to make syntax that works
- How to publish vscode extensions
- How to structure Rust projects and modules cleanly

## What's next for TuffPython
- More languages
- Reverse Python-to-TuffPython support
- Implementing the system for more programming languages (TuffRust?)

# Syntax stuff
- For aliases, the first time a non-reserved keyword is mentioned with {} after it, an alias is registered. Example: `nombre{name} = "Jay"` (The transpiled code will call `nombre` `Jay`
- Also, aliases can be used on any identifier pretty much, which includes functions and library names. This means that you can import functions and call them whatever name you want
- Translated strings: `@<language-code>" String text ... "`. Language codes are the same as the supported languages of TuffPython, and includes `en` for English. Example: `@en"Bonjour"` (Will become "Hello"). By default, it translates from whatever language you are coding in. To auto-detect a language, use `@@<language-code>" String text ... "`
