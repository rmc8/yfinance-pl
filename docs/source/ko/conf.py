import os
import sys

sys.path.insert(0, os.path.abspath("../../.."))

# Configuration file for the Sphinx documentation builder.

# -- Project information -----------------------------------------------------

project = "yfinance-pl"
copyright = "2025 rmc8"
author = "rmc8"
release = "0.7.2.0"
language = "ko"

# -- General configuration ---------------------------------------------------

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.githubpages",
    "sphinx.ext.autosectionlabel",
    "sphinx_copybutton",
]

templates_path = ["../_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------

html_title = "yfinance-pl"
html_theme = "pydata_sphinx_theme"
html_theme_options = {
    "github_url": "https://github.com/rmc8/yfinance-pl",
    "navbar_align": "left",
    "show_toc_level": 2,
    "navbar_end": ["version-switcher", "theme-switcher", "navbar-icon-links"],
    "secondary_sidebar_items": ["page-toc"],
    "switcher": {
        "json_url": "https://yfinance-pl.rmc-8.com/_static/language-switcher.json",
        "version_match": "ko",
    },
    "show_version_warning_banner": False,
}
html_static_path = ["../_static"]
html_context = {
    "default_mode": "auto",
}
