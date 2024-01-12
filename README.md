# [PyRustyMD](https://github.com/frederikbeimgraben/PyRustyMD)
> WIP! Markdown Parser written in Rust for the PixelCampus.space Wiki to support custom tags and other features.

## Features
- [ ] Parse Markdown
- [x] Parse HTML-Style Tags
- [ ] Allow only certain Tags
- [ ] Allow only certain Attributes per Tag
- [ ] markdown like additions specific to PixelCampus.space Wiki

Returns a JSON Object like:
```json
[
    {
        "id": "",
        "style": "left: 10px;",
        "classes": [
            "test",
            "abc"
        ],
        "is_self_closing": false,
        "tag": "div",
        "children": [
            "test",
            {
                "style": "",
                "children": [
                    "test2"
                ],
                "id": "spn",
                "tag": "span",
                "classes": [
                    ""
                ],
                "is_self_closing": false
            }
        ]
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