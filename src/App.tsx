import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  const [count, setCount] = useState(0)

  function executeCommands() {
    invoke('simple_command')
    invoke('command_with_message', { msg: 'ore tensai' }).then(message => {
      console.log(`command_with_message`, message)
    })
    invoke('command_with_object', { msg: { field_str: 'ore kansai', field_u32: 12 } }).then(message => {
      console.log(`command_with_object`, message)
    })
    for (let arg of [1, 2]) {
      invoke('command_with_error', { arg }).then(message => {
        console.log('command_with_error', message)
      }).catch(message => {
        console.error('command_with_error', message)
      })
    }
    invoke('async_command', { arg: 14 }).then(message => {
      console.log(`async_command`, message)
    })
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
        </p>
        <button onClick={executeCommands}>Click to execute command</button>
      </header>
    </div>
  )
}

export default App
