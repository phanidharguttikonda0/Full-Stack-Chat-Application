import React, { useState } from 'react';
import './App.css';

import { BrowserRouter, Route, Routes } from 'react-router-dom';



//* export the main contexts

export const usernameContext = React.createContext() ;
export const usernameChangeContext = React.createContext() ;

function App() {

  const [username, changeUsername] = useState("") ;



  return (
    <BrowserRouter>
      <div className="App">
          <Routes>
            <Route path='/Sign-In' />
            <Route path='/Sign-Up' />
          </Routes>
        </div>
    </BrowserRouter>
  );

}

export default App;
