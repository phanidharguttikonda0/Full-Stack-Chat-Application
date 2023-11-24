import React, { useContext, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usernameContext } from '../App';
import Chats from './Home/Chats';
import Search from './Home/Search';
import Users from './Home/Users';
import css from './css/Home.module.css';


export const searchedUserContext = React.createContext() ;
export const changeSearchedUserContext = React.createContext() ;
export const newUserAddedContext = React.createContext() ;
export const changeNewUserAddedContext = React.createContext() ;
function Home(props) {

    const username = useContext(usernameContext) ;
    const navigate = useNavigate() ;
    const [searchedUser, changeSearchedUser] = useState("") ;
    const [newUserAdded, changenewUserAdded] = useState(true) ;

    const signin = () => {
        navigate('/sign-in')
    }

    const signup = () => {
        navigate('/sign-up')
    }

    return (
        <newUserAddedContext.Provider value={newUserAdded}>
            <changeNewUserAddedContext.Provider value={changenewUserAdded}>
            <searchedUserContext.Provider value={searchedUser}>
            <changeSearchedUserContext.Provider value={changeSearchedUser}>
            <div>
            {
                username.length === 0 ? <div>
                    <button onClick={signin}> Sign In </button>
                    <button onClick={signup}> Sign Up </button>
                </div> : <div className={css.main}>
                    <div className={css.body}>
                        <div className={css.sidenav}>
                        <Search />
                        <Users />
                        </div>
                        <div className={css.chats}>
                            <Chats />
                        </div>
                        </div>
                </div>
            }
        </div>
            </changeSearchedUserContext.Provider>
        </searchedUserContext.Provider>
            </changeNewUserAddedContext.Provider>
        </newUserAddedContext.Provider>
    );
}

export default Home;