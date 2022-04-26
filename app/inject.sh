sed '/%%script%%/{
        s/%%script%%//g
        r tmp.js
}' src/template.html
