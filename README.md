# [PyRustyMD](https://github.com/frederikbeimgraben/PyRustyMD)

[![CI](https://github.com/frederikbeimgraben/PyRustyMD/actions/workflows/CI.yml/badge.svg)](https://github.com/frederikbeimgraben/PyRustyMD/actions/workflows/CI.yml/badge.svg)
[![PyPI](https://shields.io/badge/PyPI-Package-yellow?logo=python&logoColor=white&labelColor=blue)](https://pypi.org/project/pyrustymd/)

> WIP! Markdown Parser written in Rust for the PixelCampus.space Wiki to support custom tags and other features.

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/0-percent-optimized.svg)](https://forthebadge.com)

## Features
- [ ] Parse Markdown
- [x] Parse HTML-Style Tags
- [x] Allow only certain Tags
- [x] Allow only certain Attributes per Tag
- [ ] markdown like additions specific to PixelCampus.space Wiki

Returns a JSON Object like:
```json
[
    {
        "content": [
            "test",
            {
                "content": [
                    "test2"
                ],
                "tag": "span",
                "attributes": {
                    "id": "spn",
                    "class": []
                }
            }
        ],
        "tag": "div",
        "attributes": {
            "style": "left: 10px;",
            "class": [
                "test",
                "abc"
            ]
        }
    }
]
```

from this HTML:
```html
<div style="left: 10px;" class="test abc">
    test
    <span id="spn">test2</span>
</div>
```

## Usage
```python
from pyrustymd import parse

json = parse("<your html here>")
```

## Development setup

```sh
# Create virtualenv
python3 -m venv .env

# Activate virtualenv
source .env/bin/activate

# Install maturin
pip install maturin

# Build
maturin develop
```

Then just import it in your python script having the same virtualenv activated.

## Release History

* 0.1.2
    * *Add License and Readme*

* 0.1.1
    * *Changed package name and published to PyPi*

* 0.1.0
    * Basic functionality for parsing HTML-Style Tags

## Meta

Frederik Beimgraben – 
    [Website](https://beimgraben.net) |
    [GitHub](https://github.com/frederikbeimgraben)

Distributed under the GPL-3.0 license. See ``LICENSE`` for more information.

PixelCampus – 
    [Website](https://pixelcampus.space) |
    [GitHub](https://github.com/frederikbeimgraben/PixelCampus)