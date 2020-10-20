from os import path

from setuptools import setup, find_packages

# Get the long description from the README file
here = path.abspath(path.dirname(__file__))
with open(path.join(here, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name="routee-compass",
    version="0.0.1",
    description=
    "routee compass is a package for producing energy optimal routes",
    long_description=long_description,
    long_description_content_type='text/markdown',
    url="https://github.nrel.gov/MBAP/energy-optimal-routing",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Science/Research",
        "License :: Other/Proprietary License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3.6",
        "Topic :: Scientific/Engineering"
    ],
    packages=find_packages(),
    python_requires=">=3.6",
    install_requires=[
        "pandas",
        "numpy",
        "networkx",
        "scipy",
        "shapely",
    ],
    extras_require={
       "optional": [
           "osmnx",
           "geopandas",
           "sqlalchemy",
           "psycopg2",
           "requests",
       ]
    },
    author="National Renewable Energy Laboratory",
    license="Copyright ©2020 Alliance for Sustainable Energy, LLC All Rights Reserved",
)