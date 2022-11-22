/* eslint-disable no-undef */
module.exports = (variables, { tpl }) => {
  return tpl`
import { SVGProps } from 'react'
${variables.interfaces}

export interface Props {
  primaryColor: string
}

function ${variables.componentName}({ primaryColor, ...props }: SVGProps<SVGSVGElement> & Props) {
  return (
    ${variables.jsx}
  )
}

${variables.exports}
`
}
