import { render } from 'react-dom'
import * as React from 'react'
import { App, AppProps } from './index.tsx'

import('../pkg/index').then((mod) =>
  render(<App natrium={mod}></App>, document.getElementById('app-root'))
)
