# app/routes/user.py
from flask import Blueprint, jsonify

# 蓝图
example_bp = Blueprint('example', __name__)


@example_bp.route('/', methods=['GET'])
def hello_world():  # put application's code here
    return 'Hello World!'
