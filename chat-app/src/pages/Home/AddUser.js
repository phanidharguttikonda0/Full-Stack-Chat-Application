import axios from 'axios';
import React, { useContext } from 'react';
import ReactDOM from 'react-dom';
import { usernameContext } from '../../App';
import { changeNewUserAddedContext, newUserAddedContext, searchedUserContext } from '../Home';
import css from '../css/Add.module.css';
import { changeOpenContext } from './Search';

function AddUser(props) {

    const username = useContext(usernameContext) ;
    const user = useContext(searchedUserContext) ;
    const changeOpen = useContext(changeOpenContext) ;
    const newUserAdded = useContext(changeNewUserAddedContext) ;
    const value = useContext(newUserAddedContext) ;

    const add = async () => {
        console.log(`The username ${username} and the user was ${user}`)
        // here we will add the new user to the data-base
        try{
            let response = await axios.post("http://127.0.0.1:8080/search/add", {sender: username, receiver : user}) ;
                if(response.data) {
                    newUserAdded(!value) ;
                    changeOpen(false) ;
                }else{
                    alert("May be he is already in our chat-box list") ;
                    changeOpen(false) ;
                }
        }catch(err) {
            console.log(err)
        }
    }

    const close = () => {
        changeOpen(false) ;
    }

    return (
        ReactDOM.createPortal(
            <div className={css.add}>
            <button onClick={add}> Add to Check - box </button>
            <button onClick={close}> Close </button>
        </div>
            ,document.getElementById("add")
    )
    );
}

export default AddUser;

