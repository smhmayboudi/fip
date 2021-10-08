# Git Config

```shell
$ git config --global branch.autoSetupRebase always
$ git config --global color.branch true
$ git config --global color.diff true
$ git config --global color.interactive true
$ git config --global color.status true
$ git config --global color.ui true
$ git config --global commit.gpgSign true
$ git config --global core.editor "code -w"
$ git config --global difftool.code.cmd "code --wait --diff \$LOCAL \$REMOTE"
$ git config --global gpg.program gpg
$ git config --global init.defaultBranch main
$ git config --global log.date relative
$ git config --global pull.default current
$ git config --global pull.rebase true
$ git config --global push.default current
$ git config --global rebase.autoStash true
$ git config --global rerere.enabled true
$ git config --global stash.showPatch true
```

Alternative way which apply githooks folder

```shell
$ make git
```
