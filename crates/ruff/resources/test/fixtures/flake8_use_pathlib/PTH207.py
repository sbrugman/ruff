with open(filename, "w") as f:
    f.write("hello world")

with open(filename, mode="w") as f:
    f.write("hello world")

with open(filename, "wb") as f:
    f.write(b"hello world")

with open(filename, "w", encoding="utf-8") as f:
    f.write(b"hello world")

with Path(file_name).open("w") as f:
    f.write("hello world")

with Path(file_name).open("wb") as f:
    f.write(b"hello world")

with Path(file_name).open("r") as f:
    content = f.read().readlines()

# Sourced from: https://github.com/search?q=with+open%28%29+.read%28%29+language%3APython&type=code
def func():
    with open(output_filename, "rb") as fp:
        return fp.read()

with Path(CURRENT_DIR, 'templates', 'technical_404.html').open() as fh:
    t = DEBUG_ENGINE.from_string(fh.read())

with Path("Doc/sphinx-warnings.txt").open() as f:
    warnings = f.read().splitlines()

# No match unless type of `file_name` is known (?)
with file_name.open() as f:
    data = f.read()


with Path(
    "./bazel-execroot/external/bazel_cc_toolchain/"
    "clang_detected_variables.bzl"
).open() as f:
    clang_vars = f.read()

with open(os.path.join(source_path, fname)) as f:
    content = f.read()
    
with open(filename) as f:
    title = f.readline()[2:]  # removing markdown title "# "

# No match: not builtins `open`
class Hello:
    def func(self):
        with self.open('r', encoding, *args, **kwargs) as strm:
            return strm.read()

# No match: other than text
with pathlib.Path(fname).open() as f:
    for n, cols in enumerate(csv.reader(f)):
        if n == 0:
            continue  # header

with open(_TF_BAZELRC, 'a') as f:
    f.write(line + '\n')