# simulo-template

Setup:

Add this library as a submodule in your CMake project:

```
git submodule add https://github.com/richgrov/libsimulo.git
git submodule update --init --recursive
```

Update your `CMakeLists.txt`:

```cmake
add_subdirectory(libsimulo)
target_link_libraries(my_program PRIVATE simulo)
```
