# What is Vladislav2?
A fast, lightweight and configurable, multi-threaded html preprocessor inspired by [Sergey](https://sergey.cool/).
A rewrite of my original [vladislav](https://github.com/54696d654a6f6c74/vladislav) in Rust.

# Usage
### Getting started
A blank run of the `driver.py` file will generate a `vlad_settings.json` file than can be edited to configure Vladislav.

### The features
By default Vladislav looks for `.vlad` files to process and output into the `vlad_out` folder.

A `.vlad` file is a regular HTML file but it's expected to contain `include` comments.

### Include comments
#### Syntax
```<!-- include -[FLAGS] <PATH> -->```

`-[FLAGS]` can be omitted, more information on those later.
`<PATH>` specifies the path to a `.vlad` file (a template).

**Note: The path must be either relative to the project root, or absolute!**

#### Flags
Currently only the `r` flag is available. It signifies a recursive include.

# Examples
### Basic:
templates/navbar.vlad
```html
<nav>
  <button id="1" class="nav-btn"> Don't </button>
  <button id="2" class="nav-btn"> hurt </button>
  <button id="3" class="nav-btn"> me </button>
</nav>
```

index.vlad
```html
<html>
  <header>
    <!-- include templates/navbar.vlad -->
  </header>
</html>
```

Run `python driver.py` and you get:

vlad_out/index.html
```html
<html>
  <header>
    <nav>
      <button id="1" class="nav-btn"> Don't </button>
      <button id="2" class="nav-btn"> hurt </button>
      <button id="3" class="nav-btn"> me </button>
    </nav>
  </header>
</html>
```

### Recursive

templates/header.vlad
```html
<head>
  <meta charset="UTF-8">
</head>
<header>
  <!-- include templates/navbar.vlad -->
</header>
```

index.vlad
```html
<html>
  <!-- include -r templates/header.vlad -->
  <body>
    <h1> No more! </h1>
  </body>
</html>
```

Run `python driver.py` and you get:
```html
<html>
<head>
  <meta charset="UTF-8">
</head>
<header>
  <nav>
    <button id="1" class="nav-btn"> Don't </button>
    <button id="2" class="nav-btn"> hurt </button>
    <button id="3" class="nav-btn"> me </button>
  </nav>
</header>
<body>
    <h1> No more! </h1>
  </body>
</html>
```
