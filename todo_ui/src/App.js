import React, { Component } from 'react';
import axios from 'axios';

import ToDoItem from './components/ToDoItem';
import CreateToDoItem from './components/CreateToDoItem';

import "./App.css";

class App extends Component {
	state = {
		"pending_items": [],
		"done_items": [],
		"pending_items_count": 0,
		"done_items_count": 0
	}
	// make the API call
	getItems() {
		axios.get("http://127.0.0.1:8080/v1/item/get", { headers: { "token": "some_token" } })
			.then(response => {
				let pending_items = response.data["pending_items"]
				let done_items = response.data["done_items"]
				this.setState({
					"pending_items": this.processItemValues(pending_items),
					"done_items": this.processItemValues(done_items),
					"pending_item_count": response.data["pending_item_count"],
					"done_item_count": response.data["done_item_count"]
				})
			})
	}
	// ensures the API call is updated when mounted
	componentDidMount() {
		this.getItems();
	}
	// convert items from API to HTML
	processItemValues(items) {
		let itemList = [];
		items.forEach((item, _) => {
			itemList.push(
				<ToDoItem key={item.title + item.status} title={item.title}
					status={item.status} passBackResponse={this.handleReturnedState} />
			)
		});
		return itemList
	}

	handleReturnedState = (response) => {
		let pending_items = response.data["pending_items"]
		let done_items = response.data["done_items"]
		this.setState({
			"pending_items": this.processItemValues(pending_items),
			"done_items": this.processItemValues(done_items),
			"pending_item_count": response.data["pending_item_count"],
			"done_item_count": response.data["done_item_count"]
		})
	}
	// returns the HTML to be rendered
	render() {
		return (
			<div className='App'>
				<div className='mainContainer'>
					<div className='header'>
						<p>Complete tasks: {this.state.done_items_count}</p>
						<p>Pending tasks: {this.state.pending_items_count}</p>
					</div>
					<h1>Done Items</h1>
					{this.state.done_items}
					<h1>Pending Items</h1>
					{this.state.pending_items}
					<CreateToDoItem passBackResponse={this.handleReturnedState} />
				</div>
			</div>
		)
	}
}

export default App;