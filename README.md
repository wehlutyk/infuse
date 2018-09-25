Infuse
======

This project aims to create a pdf-processing Rust library, à la [Grobid](https://github.com/kermitt2/grobid), which can be used to read scientific pdfs as if they were normal web pages. It will then be integrated in a webapp by compiling the whole thing to Wasm.

Status
------

This project is still embryonic.

Reading pdfs works, in the browser also.

Current work is going into finding the proper formulation of the problem that consists in piecing the various objects encoded in the pdf to reconstruct the full body text, while also classifying those pieces into the various types of content (footnote, caption, metadata, body, ...) there is in the pdf.

Read more in the [issues](https://github.com/wehlutyk/infuse/issues)!
