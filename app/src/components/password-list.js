import React from 'react';
import './password-list.css'

import { Button } from './button';

export class PasswordList extends React.Component {
    render() {
        return (
            <div className="password-list">
                <header className="search-box">
                    <input className="main-search-box" placeholder="Search for passwords" />
                    <Button>
                        <img src="icons/add.svg" alt="" />
                    </Button>
                </header>

                <h1 className="category-title">Work</h1>
                <div className="category">
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    </div>

                    <h1 className="category-title">Games</h1>
                    <div className="category">
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    </div>

                    <h1 className="category-title">Personal</h1>
                    <div className="category">
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                    <div className="password">
                        <h1 className="password-title">Password name</h1>
                        <p className="password-description">email@domain.com</p>
                        <img className="main-button password-icon" src="icons/add.svg" alt="" />
                    </div>
                </div>
            </div>
        );
    }
}