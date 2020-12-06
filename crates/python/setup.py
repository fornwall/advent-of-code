from os import path

from setuptools import setup
from setuptools_rust import Binding, RustExtension

# Read the contents of the README file:
this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, "README.md"), encoding="utf-8") as f:
    long_description = f.read()

setup(
    name="advent-of-code",
    url="https://github.com/fornwall/advent-of-code",
    long_description=long_description,
    long_description_content_type='text/markdown',
    version="2019.12.190",
    rust_extensions=[
        RustExtension("advent_of_code", "Cargo.toml", binding=Binding.PyO3)
    ],
    packages=["cli"],
    entry_points={
        'console_scripts': ['advent-of-code-py=cli.main:main'],
    },
    test_suite="tests",
    zip_safe=False,
)
