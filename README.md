Infuse
======

This project aims to create a pdf-processing Rust library, à la [Grobid](https://github.com/kermitt2/grobid), which can be used to read scientific pdfs as if they were normal web pages. It will then be integrated in a webapp by compiling the whole thing to Wasm.

The implementation is still embryonic. But there is an interesting [presentation](https://www.youtube.com/watch?v=_LuoPps0TJU) (37m talk, 18m questions), and associated [slides](https://wehlutyk.github.io/infuse-presentation)!

Status
------

Reading pdfs works, in the browser also.

Current work is focused on piecing together the various objects encoded in the pdf in orderto reconstruct the tree of content, including full body text, while also classifying those pieces into the various types we're interested in (footnote, caption, metadata, body, ...).

Read more in the [issues](https://github.com/wehlutyk/infuse/issues)!
