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

[![Website](
    https://img.shields.io/badge/My_Website-Beimgraben.NET-orange?logo=firefox&logoColor=orange&labelColor=white&style=social
)](https://beimgraben.net)
[![GitHub followers](
    https://img.shields.io/github/followers/frederikbeimgraben?label=Follow&style=social
)](https://github.com/frederikbeimgraben/)
[![PGP](
    https://img.shields.io/badge/PGP-0x9D6D6D6C-blue?logo=monkeytie&logoColor=black&labelColor=white&style=social
)](https://keybase.io/beimgraben/pgp_keys.asc)

[![PixelCampus](
    https://img.shields.io/badge/-PixelCampus.Space-blue?logo=minetest&logoColor=blue&labelColor=white&style=social
)](https://pixelcampus.space)
[![PixelCampus](
    https://img.shields.io/badge/-PixelCampus-white?logo=github&logoColor=black&style=social
)](https://github.com/frederikbeimgraben/PixelCampus)

## License

Distributed under the GPL-3.0 license. See ``LICENSE`` for more information.