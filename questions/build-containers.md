# prompt
How many containers are involved in the shining build?

# options
- [ ] 0, the build happens on the host.
- [ ] 1, the sandbox.
- [x] 2, the build container and the runtime container.
- [ ] 3, the build container, the runtime container, the sandbox.

# hint
no hints here, sorry

# explaination
There are two containers during the build.

The build container which complies our sources.

The runtime container in which the compiled targets are place in, a.k.a the sandbox.
