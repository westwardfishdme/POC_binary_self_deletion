# Self-deleting binary (happytime)

The code in this repository is an example of what a self-deleting binary in linux would look like.
You can run it with cargo-- or compile it and then run it. Designed for linux machines, should work
on any unix-like system. The live branch contains add-ons for malicious code execution, which is currently
a work in progress. The main branch only contains a theoretical program that could find where the binary 
is being ran from for self-deletion.

## How does this work?
In unix-like systems, the OS will create a symlink to the binary at `/proc/{process_id}/exe`.
What this program does is it creates a thread to keep track of the location of the binary, and after a set period
of time (30 seconds) it will delete the binary.

In a real world scenario, this algorithm could be implemented by an attacker to either rotate the name
of a binary or delete it. Ideally, it would be used as a module for the attacker's malware, and then once their malware executes-- 
delete the binary with a supplied method.

### Notes:

- Any conditional can be used to be set to delete the binary.
- It was inspired by [Vault-8's Hive self-deletion algorithm](https://wikileaks.org/vault7/document/hive-DevelopersGuide/hive-DevelopersGuide.pdf) which was
built and developed by the CIA to hide traces of their malware from the victim's
computer. It never left experimentation, but it was kept around for potential future
use.

## Disclaimer
This code was published for educational purposes only. I am not responsible for any
use of this algorithm outside of the intended scope.
