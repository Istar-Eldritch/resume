# This is my resume

### Local development
The project currently uses [trunk-rs](https://trunkrs.dev/). You can get the app running locally with:
```bash
trunk serve
```


### Deployment

Switch to the gh-pages branch and run:
```bash
trunk build --release
mv dist docs
```
Commit the docs folder and push it. The resume is hosted using github pages.

Beware, the build config file in the gh-pages is different than in your main branch:
```toml
# Trunk.toml
[build]
dist = "docs"
public_url = "/resume/"
```

### Creating a PDF

There is currently no functionality to export PDF on this project, but you can access a PDF version using [`sedja.com`](https://www.sejda.com/html-to-pdf?save-link=https://istar-eldritch.github.io/resume/?open=true)
It makes use of the `open=true` query argument, that expands all the roles and education and simply creates a PDF from the website.