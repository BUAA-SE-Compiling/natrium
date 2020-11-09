import * as React from 'react'
import MonacoEditor from 'react-monaco-editor'
import './index.styl'

export interface Natrium {
  compile: (string) => string
}

export interface AppProps {
  natrium: Natrium
}

export interface AppState {
  error?: string
  code: string
  compiledCode?: string
}

export class App extends React.Component<AppProps, AppState> {
  natrium: Natrium

  constructor(props: AppProps) {
    super(props)
    this.natrium = props.natrium
    this.state = {
      error: undefined,
      code: '// code here',
      compiledCode: undefined,
    }
  }

  onCodeUpdate(code: string) {
    this.setState({ code: code })
  }

  render() {
    return (
      <div className="app">
        <div className="editor-space">
          <Editor code={this.state.code} onCodeChange={(code) => this.onCodeUpdate(code)}></Editor>
        </div>
        <div className="result-space">
          <button onClick={() => this.compile(this.state.code)}> Compile</button>
          {this.state.compiledCode && <pre>{this.state.compiledCode}</pre>}
          {this.state.error && <pre>{this.state.error}</pre>}
        </div>
      </div>
    )
  }

  compile(code: string) {
    try {
      let compiledCode = this.natrium.compile(code)
      this.setState({ compiledCode: compiledCode })
    } catch (e) {
      this.setState({ error: e, compiledCode: undefined })
    }
  }
}

interface EditorProps {
  onCodeChange: (code: string) => void
  code: string
}

class Editor extends React.Component<EditorProps> {
  constructor(props: EditorProps) {
    super(props)
    this.props = props
  }

  props: EditorProps = undefined

  updateCode(code: string) {
    this.props.onCodeChange(code)
  }

  render() {
    return (
      <MonacoEditor
        language="javascript"
        theme="vs-dark"
        value={this.props.code}
        onChange={(code) => this.updateCode(code)}
        options={{ automaticLayout: true }}
      ></MonacoEditor>
    )
  }
}
