import axios from 'axios';
import React, { useContext, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usernameChangeContext } from '../../App';
import css from './authentication.module.css';

function SignIn(props) {

    const [username, changeUsername] = useState("") ;
    const [password, changePassword] = useState("") ;
    const usernamechange = useContext(usernameChangeContext) ;
    const navigate = useNavigate() ;

    const onsubmit = async () => {
        try{
            let response = (await axios.post("http://127.0.0.1:8080/sign-in", {
                username: username,
                password: password
            })) ;
            console.log(response)
            if (response.data) {
                usernamechange(username) ;
                navigate('/')
            }
            else alert("Invalid Credentials")
        }catch(err){
            alert("Some Error caused from the server side")
        }
    }

    const nav = () => {
        navigate('/sign-up') ;
    }

    return (
        <div className={css.sign}>
            <div className={css.main}>
                <input type='text' placeholder='username' value={username} onChange={(event)=> changeUsername(event.target.value)} />
                <input type='password' placeholder='password' value={password} onChange={(event) => changePassword(event.target.value)} />
                <button className={css.link} onClick={nav}> sign up </button>
                <button className={css.btn} onClick={onsubmit}> sign in </button>
            </div>
        </div>
    );
}

export default SignIn;