import { transform  } from '@svgr/core'

function createComponent(svg: string) {
  return transform(svg, {
    replaceAttrValues: { '#6c63ff': '{primaryColor}' },
    svgProps:          { style: '{styleProps}' }
  })
}

import selectionSvg from './selection.svg'
export const Selection = createComponent(selectionSvg)
