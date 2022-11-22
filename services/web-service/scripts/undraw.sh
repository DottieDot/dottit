
npx @svgr/cli \
  --out-dir ./src/undraw \
  --ignore-existing \
  --typescript \
  --no-dimensions \
  --icon \
  --template ./scripts/svgTemplate.js \
  --replace-attr-values "#6c63ff"="{primaryColor}" \
  -- ./src/undraw
npx eslint ./src/undraw --ext .ts,.tsx --fix