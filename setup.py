"""
Setup script for building Sentience Core Python extension using maturin.
"""

from setuptools import setup

setup(
    name="sentience-core",
    version="0.2.0",
    description="SRAI-compliant Sentience Core with Python bindings",
    author="SRAI Team",
    author_email="team@srai.ai",
    url="https://github.com/srai/sentience",
    # Python packages
    packages=["sentience_core"],
    package_dir={"sentience_core": "python/sentience_core"},
    # Dependencies
    install_requires=[
        "numpy>=1.21.0",
    ],
    # Development dependencies
    extras_require={
        "dev": [
            "pytest>=7.0",
            "black>=22.0",
            "mypy>=1.0",
        ]
    },
    # Metadata
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Rust",
    ],
    python_requires=">=3.8",
    # Include README
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
)
