cmake_minimum_required(VERSION 3.28)   # or newer
project(fibcpp LANGUAGES CXX)

add_library(fibcpp fib.cpp)

include(GNUInstallDirs)        # <── defines CMAKE_INSTALL_INCLUDEDIR, LIBDIR, BINDIR …

install(TARGETS fibcpp                       # the static/ shared library you already add_library()
        ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})  # for future executables

# Optional: ship public headers so bindgen/cxx can find them
install(DIRECTORY ${PROJECT_SOURCE_DIR}/include/
        DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})