from os import path

from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip

# Read the contents of the README file:
this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, "README.md"), encoding="utf-8") as f:
    long_description = f.read()

setup(
    name="advent-of-code",
    url="https://github.com/fornwall/advent-of-code",
    long_description=long_description,
    long_description_content_type="text/markdown",
    version="2024.20.0",
    rust_extensions=[
        RustExtension(
            "advent_of_code", binding=Binding.PyO3, strip=Strip.All, py_limited_api=True
        )
    ],
    packages=["advent_of_code", "advent_of_code_cli"],
    package_data={"advent_of_code": ["__init__.pyi", "py.typed"]},
    entry_points={
        "console_scripts": ["advent-of-code-py=advent_of_code_cli.main:main"],
    },
    test_suite="tests",
    zip_safe=False,
)
