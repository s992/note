INSTALL_DIR=$1

if [ -z "$INSTALL_DIR" ]; then
  INSTALL_DIR="$HOME/dotfiles/bin"
fi

bazel build //src:note
cp bazel-bin/src/note "$INSTALL_DIR"
