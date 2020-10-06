import React, { Component } from "react";
import ReactDOM from "react-dom";

class App extends Component {
    constructor() {
        super();
    }

    render() {
        return (
            <div>
                <h1>Hello Rustaceans!</h1>
            </div>
        );
    }
}

export default App;

const wrapper = document.getElementById("App");
wrapper ? ReactDOM.render(<App />, wrapper) : false;