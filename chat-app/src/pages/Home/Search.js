import axios from 'axios';
import React, { useContext, useEffect, useState } from 'react';
import { changeSearchedUserContext } from '../Home';
import css from '../css/Search.module.css';
import AddUser from './AddUser';

export const changeOpenContext = React.createContext() ;
function Search(props) {

    const [username, changeusername] = useState("") ;
    const changeSearchedUser = useContext(changeSearchedUserContext) ;
    const [close, open] = useState(false) ;

    useEffect(()=>{}, [close]) ;

    const onSearch = async () => {
        if(username.length > 3) {

            let response = await axios.post("http://127.0.0.1:8080/search", {
                username: username
            }) ;

            if(response.data) {
                changeSearchedUser(username) ;
                open(true) ;
            }else{
                alert('The user doesn\'t exists')
            }
        }else{
            alert('invalid username min length > 3') ;
        }
    }

    return (
            <changeOpenContext.Provider value={open}>
            <div className={css.search}>
                <input type='text' placeholder='Enter Username' value={username} onChange={(event) => {changeusername(event.target.value)}} />
                <button onClick={onSearch}> Search </button>
                {
                    close ? <AddUser /> : undefined
                }
            </div>
            </changeOpenContext.Provider>
    ) ;
    
}

export default Search;

