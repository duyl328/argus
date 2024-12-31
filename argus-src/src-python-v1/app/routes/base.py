# app/routes/user.py
import sys

from flask import Blueprint, jsonify

# 蓝图
base_bp = Blueprint('base', __name__)


@base_bp.route('/exit', methods=['GET'])
def exit_app():
    sys.exit("调用者结束进程 !")  # 终止 Flask 应用的进程
