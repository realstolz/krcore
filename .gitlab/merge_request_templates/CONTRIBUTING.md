Contributing to Mitosis

**Check list**

- [ ] Make sure you fixed all the warnings generated by the rust compile. If not, specific why the warning exists in the merge request.

- [ ] Add brief comments to describe what you are doing

- [ ] Each single function should be written in a single `fn test_xxx()`, with comments describing what is testing and what is the pre-request (e.g., `init_ctx`) of this function.

- [ ] If you update the submodule, be sure to run `git submodule update --init --recursive --remote` to update the references.
