from flask import Flask, request, jsonify
import json
from coloured_json import pretty_print_json_colored

app = Flask(__name__)
# app.config['JSONIFY_PRETTYPRINT_REGULAR'] = True


# Route implementation
@app.route('/axis/non-dmz/api/PPIM/v1/update-customer-limit', methods=['POST'])
def update_customer_limit():
    required_headers = [
        "x-fapi-channel-id",
        "x-fapi-epoch-millis",
        "x-fapi-uuid",
        "x-fapi-serviceId",
        "x-fapi-serviceVersion",
        "X-IBM-Client-Id",
        "X-IBM-Client-Secret"
    ]

    # Check required headers
    missing_headers = [h for h in required_headers if h not in request.headers]
    if missing_headers:
        return jsonify({
            "error": f"Missing required headers: {', '.join(missing_headers)}"
        }), 400

    # Log received JSON for debugging
    req_json = request.get_json()
    print("Received JSON:")
    pretty_print_json_colored(req_json)

    # Build mock response
    response = {
        "Data": {
            "Resp_Code": "00",
            "Cumulative_Bal": "8000.00",
            "Avail_Limit": "2000.00"
        },
        "Risk": {}
    }

    print("Response JSON:")
    pretty_print_json_colored(response)

    return jsonify(response), 200

@app.route('/axis/non-dmz/api/PPIM/v1/add-customer', methods=['POST'])
def add_customer():
    required_headers = [
        "x-fapi-channel-id",
        "x-fapi-epoch-millis",
        "x-fapi-uuid",
        "x-fapi-serviceId",
        "x-fapi-serviceVersion",
        "X-IBM-Client-Id",
        "X-IBM-Client-Secret"
    ]

    # Check for missing headers
    missing_headers = [h for h in required_headers if h not in request.headers]
    if missing_headers:
        return jsonify({
            "error": f"Missing required headers: {', '.join(missing_headers)}"
        }), 400

    # Get JSON request
    req_json = request.get_json()
    print("Received JSON:")
    pretty_print_json_colored(req_json)

    # Build mock response similar to the swagger example
    response = {
        "Data": {
            "Resp_Code": "000",
            "Unique_Id": "515427983121234",
            "KYC_Flag": "P",
            "KYC_Updated_Channel": "PGO",
            "KYC_Updated_On": "01-01-2018 11:00:00"
        },
        "Risk": {},
        "Links": {},
        "Meta": {}
    }

    print("Response JSON:")
    pretty_print_json_colored(response)

    return jsonify(response), 200


if __name__ == '__main__':
    # For HTTPS with self-signed cert (comment this out for HTTP)
    # app.run(port=8443, ssl_context=('cert.pem', 'key.pem'))

    # For simple HTTP (easier for initial testing)
    app.run(host='0.0.0.0', port=5000, debug=True, ssl_context=('cert.pem', 'key.pem'))
