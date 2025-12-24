# AwsDSL Overview

A simple DSL to create resources in AWS. A recursive descent parser is used for
parsing and Rust SDK is used to make the necessary API calls.

At present, in any infrastructure stack of considerable size, many different
languages and tools are used with no visibility of each other runtime.
The objective is to perform all the necessary actions in an AWS stack
using one DSL: from AWS infrastructure deployments to OS manipulations,
and finally application configurations.
