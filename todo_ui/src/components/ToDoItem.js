import React, { Component } from "react";
import axios from "axios";

import "../App.css";

class ToDoItem extends Component {
	state = {
		"title": this.props.title,
		"status": this.props.status,
		"button": this.processStatus(this.props.status)
	}

	processStatus(status) {
		return status == "PENDING" ? "edit" : "delete"
	}

	inverseStatus(status) {
		return status == "PENDING" ? "DONE" : "PENDING"
	}

	sendRequest = () => {
		axios.post("http://127.0.0.1:8080/v1/item/" + this.state.button,
			{
				"title": this.state.title,
				"status": this.inverseStatus(this.state.status)
			},
			{ headers: { "token": "some_token" } })
			.then(response => {
				this.props.passBackResponse(response);
			});
	}

	render() {
		return (
			<div className="itemContainer">
				<p>{this.state.title}</p>
				<div className="actionButton" onClick={this.sendRequest}>{this.state.button}</div>
			</div>
		)
	}
}

export default ToDoItem;