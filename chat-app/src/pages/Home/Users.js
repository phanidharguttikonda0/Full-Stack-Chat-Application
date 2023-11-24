import axios from 'axios';
import React, { useContext, useEffect, useState } from 'react';
import { usernameContext } from '../../App';
import { newUserAddedContext } from '../Home';
import css from '../css/Users.module.css';
import Chats from './Chats';

function Users(props) {

    const [users, changeUsers] = useState([]) ;
    const isnewUserAdded = useContext(newUserAddedContext) ;
    const username = useContext(usernameContext) ;

    useEffect(() => {
        // fetching the users
        const fetchUsers = async () => {
            const response = await axios.post("http://127.0.0.1:8080/users", {username: username}) ;
            console.log(response.data) ;
            changeUsers(response.data) ;
        }
        fetchUsers() ;
    }, [isnewUserAdded]) ; // now every time a new user is added it will render over here

    return (
        <div className={css.users}>
            {
                users.map((item, index) => {
                    return <div className={css.user} key={index} onClick={<Chats user={item}/>}>
                        {item}
                        </div>
                })
            }
        </div>
    );
}

export default Users;