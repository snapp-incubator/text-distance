# Text-distance
Text-distance is a python package written in rust to calculate similarity of two texts. currently supports below algorithms:

1. **Bi-gram jaccard similarity**
2. **Cosine similarity**
3. **longest common subsequence**
4. **longest common substring**



## installation 

to install python package from source:

`pip install maturin`

`maturin build --release`

it will create a whl file under target folder and then you can just install the whl file in python

example usage:

```python
import textdistance
cosine_distance = textdistance.cosine_similarities(["hello there"],
                                           [["hello there", "hi there"]])

jaccard_distance = textdistance.jaccard_similarities(["hello there"],
                                           [["hello there", "hi there"]])

lcsseq = textdistance.longest_common_subsequence_max(["hello there"],
                                           [["hello there", "hi there"]])

lcstr = textdistance.longest_common_substring_max(["hello there"],
                                           [["hello there", "hi there"]])
```



