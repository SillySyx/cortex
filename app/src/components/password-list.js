import React from 'react';
import './password-list.css'

import { test } from 'cortexWasm';

import { Button } from './button';

export class PasswordList extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            categories: loadCategories(),
        };
    }

    render() {
        return (
            <div className="password-list">
                <header className="search-box">
                    <input className="main-search-box" placeholder="Search for passwords" />
                    <Button>
                        <img src="icons/add.svg" alt="" />
                    </Button>
                </header>

                {this.state.categories.map((category, index) => (
                    <div key={index}>
                        <h1 className="category-title">{category.title}</h1>
                        <div className="category">
                        {category.passwords.map(password => (
                            <div key={password.id} className="password">
                                <h1 className="password-title">{password.name}</h1>
                                <p className="password-description">{password.desc}</p>
                                <img className="main-button password-icon" src="icons/add.svg" alt="" />
                            </div>
                        ))}
                        </div>
                    </div>
                ))}
            </div>
        );
    }
}

function loadPasswords() {
    // localStorage.setItem("passwords", JSON.stringify([1,2,3]));
    // const data = JSON.parse(localStorage.getItem("passwords"));

    console.log(test());
    // load password list from webassembly (decrypts bytes and returns password list)

    return [
        {
            id: "1",
            category: "Games",
            name: "Black desert",
        },
        {
            id: "2",
            category: "Games",
            name: "Black desert2",
        },
        {
            id: "3",
            category: "Work",
            name: "Atea",
            desc: "kristoffer.hagelstam@atea.com",
        },
        {
            id: "4",
            category: "Personal",
            name: "outlook",
            desc: "kristoffer.hagelstam@outlook.com",
        },
    ];
}

function loadCategories() {
    const passwords = loadPasswords();

    return passwords
        .sort((a, b) => a.category.localeCompare(b.category))
        .reduce((categories, password) => {
            let category = categories.find(c => c.title === password.category);
            if (!category) {
                category = {
                    title: password.category,
                    passwords: [],
                };
                categories.push(category);
            }

            category.passwords.push({
                id: password.id,
                name: password.name,
                desc: password.desc,
            });

            return categories;
        }, []);
}