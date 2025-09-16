#!/usr/bin/env python3
"""Simple authentication server for testing RustFlix frontend"""

from flask import Flask, request, jsonify
from flask_cors import CORS
import uuid

app = Flask(__name__)
CORS(app)

@app.route('/api/auth/login', methods=['POST'])
def login():
    data = request.get_json()
    username = data.get('username', '')
    password = data.get('password', '')
    
    # Simple hardcoded authentication
    if (username == 'admin' and password == 'password123') or \
       (username == 'test' and password == 'test123'):
        return jsonify({
            'token': 'test-jwt-token',
            'user': {
                'id': str(uuid.uuid4()),
                'username': username,
                'email': 'admin@example.com',
                'role': 'admin'
            }
        })
    else:
        return jsonify({'error': 'Invalid credentials'}), 401

@app.route('/api/auth/register', methods=['POST'])
def register():
    data = request.get_json()
    username = data.get('username', '')
    email = data.get('email', '')
    password = data.get('password', '')
    
    if username and email and len(password) >= 6:
        return jsonify({
            'token': 'test-jwt-token',
            'user': {
                'id': str(uuid.uuid4()),
                'username': username,
                'email': email,
                'role': 'user'
            }
        })
    else:
        return jsonify({'error': 'Invalid input'}), 400

@app.route('/api/auth/me', methods=['GET'])
def get_current_user():
    return jsonify({
        'id': str(uuid.uuid4()),
        'username': 'admin',
        'email': 'admin@example.com',
        'role': 'admin'
    })

@app.route('/health', methods=['GET'])
def health():
    return 'OK'

if __name__ == '__main__':
    print("Starting simple auth server on port 8080...")
    print("Test credentials:")
    print("  Username: admin, Password: password123")
    print("  Username: test, Password: test123")
    app.run(host='0.0.0.0', port=8080, debug=True)
