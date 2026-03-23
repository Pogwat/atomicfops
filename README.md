# Atomic File Operations (AtomicFOps)
A collection of wrappers around the std file operations like write, remove_file ext..
Since the rename syscall is atomic as in it either succeded or failed no in beetwen atomicfops jsut wraps abunch of file operations with rename, temproary files are operated on and switched in with rename so theoretically no in-beetwen state is possible
Most of these wrappers are generated using a macro
