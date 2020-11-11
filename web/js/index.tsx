import * as React from 'react'
import './index.styl'
import * as Natrium from '../pkg/index'
import AceEditor from 'react-ace'

export interface AppProps {
  natrium: typeof Natrium
}

export interface AppState {
  error?: string
  code: string
  compiledCode?: string
  output: string
}

export class App extends React.Component<AppProps, AppState> {
  natrium: typeof Natrium

  constructor(props: AppProps) {
    super(props)
    this.natrium = props.natrium
    this.state = {
      error: undefined,
      code: '',
      compiledCode: undefined,
      output: '',
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
          <button onClick={() => this.compile(this.state.code)}>Compile</button>
          <button onClick={() => this.run(this.state.code)}>Run</button>
          {this.state.compiledCode && <pre>{this.state.compiledCode}</pre>}
          {this.state.error && <pre>{this.state.error}</pre>}
          {this.state.output && <pre>{this.state.output}</pre>}
        </div>
      </div>
    )
  }

  compile(code: string) {
    try {
      let compiledCode = this.natrium.compile(code)
      this.setState({ compiledCode: compiledCode, output: '' })
    } catch (e) {
      this.setState({ error: e, compiledCode: undefined })
    }
  }

  run(code: string) {
    try {
      this.setState({ output: '', compiledCode: undefined })
      this.natrium.run(
        code,
        () => '',
        (x: Uint8Array) => this.appendCode(x)
      )
      console.log('finished')
    } catch (e) {
      this.setState({ error: e, compiledCode: undefined })
    }
  }

  appendCode(x: Uint8Array) {
    let s = new TextDecoder('utf8').decode(x)
    console.log('out', s)
    this.setState((x) => ({
      output: x.output + s,
    }))
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
      <AceEditor
        className="ace-editor"
        mode="rust"
        value={this.props.code}
        onChange={(code) => this.updateCode(code)}
        width="100%"
        height="100%"
        fontSize="1rem"
        placeholder="// Your code here"
        editorProps={{
          $blockScrolling: true,
        }}
      ></AceEditor>
    )
  }
}
