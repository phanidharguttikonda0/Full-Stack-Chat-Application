import React, { useState } from 'react';
import './App.css';

import { BrowserRouter, Route, Routes } from 'react-router-dom';
import Home from './pages/Home';
import AddUser from './pages/Home/AddUser';
import SignIn from './pages/LoginComponents/SignIn';
import SignUp from './pages/LoginComponents/SignUp';



//* export the main contexts

export const usernameContext = React.createContext() ;
export const usernameChangeContext = React.createContext() ;

function App() {

  const [username, changeUsername] = useState("") ;



  return (
    <BrowserRouter>
      <usernameChangeContext.Provider value={changeUsername}>
        <usernameContext.Provider value={username}>
          <div className="App">
            <Routes>
              <Route path='/sign-in' element={<SignIn />} />
              <Route path='/sign-up' element={<SignUp />} />
              <Route path='/' element={<Home />} />
              <Route path='/add' element={<AddUser/>} />
            </Routes>
          </div>
        </usernameContext.Provider>
      </usernameChangeContext.Provider>
    </BrowserRouter>
  );

}

export default App;
