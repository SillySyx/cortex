export class WebRtcService {
    constructor(id) {
        this.socket = new WebSocket("wss://webrtc-signal.azurewebsites.net");

        const configuration = {
            // iceServers: [
            //     { urls: "stun.bahnhof.net:3478" },
            //     { urls: "stun.l.google.com:19302" },
            //     { urls: "stun1.l.google.com:19302" },
            //     { urls: "stun2.l.google.com:19302" },
            //     { urls: "stun3.l.google.com:19302" },
            //     { urls: "stun4.l.google.com:19302" },
            // ],
        };
        this.rtc = new RTCPeerConnection(configuration);

        this.connected = () => {};
        this.disconnected = () => {};
        this.message = msg => {};

        this.setupWebSocket(id);
        this.setupWebRtc();
    }

    closeConnection() {
        this.socket.close();
        this.rtc.close();
    }

    setupWebSocket(id) {
        this.socket.onmessage = event => {
            const message = JSON.parse(event.data);

            if (message.type === "make_offer") {
                this.makeOffer();
            }
            if (message.type === "make_answer") {
                this.makeAnswer(message.data);
            }
            if (message.type === "accept_answer") {
                this.acceptAnswer(message.data);
            }
        };
        this.socket.onopen = _ => {
            this.socket.send(JSON.stringify({
                type: "connect",
                data: id,
            }));
        };
    }

    setupWebRtc() {
        this.rtc.ondatachannel = event => {
            this.setupWebRtcChannel(event.channel);
        };
    }

    setupWebRtcChannel(channel) {
        this.rtc.channel = channel;
        this.rtc.channel.onmessage = event => {
            this.message(event.data);
        };
        this.rtc.channel.onopen = () => {
            this.socket.close();
            this.connected();
        };
        this.rtc.channel.onclose = () => {
            this.disconnected();
        };
    }

    async makeOffer() {
        const channel = this.rtc.createDataChannel("data");
        this.setupWebRtcChannel(channel);

        let timeout = null;
        this.rtc.onicecandidate = event => {
            if (!event.candidate) {
                if (timeout) {
                    clearTimeout(timeout);
                }

                timeout = setTimeout(() => {
                    this.socket.send(JSON.stringify({
                        type: "offer",
                        data: this.rtc.localDescription,
                    }));
                }, 100);
            }
        };

        const offer = await this.rtc.createOffer();
        await this.rtc.setLocalDescription(offer);
    }

    async makeAnswer(offer) {
        await this.rtc.setRemoteDescription(offer);

        let timeout = null;
        this.rtc.onicecandidate = event => {
            if (!event.candidate) {
                if (timeout) {
                    clearTimeout(timeout);
                }

                timeout = setTimeout(() => {
                    this.socket.send(JSON.stringify({
                        type: "answer",
                        data: this.rtc.localDescription,
                    }));
                }, 100);
            }
        };
        
        const answer = await this.rtc.createAnswer();
        await this.rtc.setLocalDescription(answer);
    }

    async acceptAnswer(answer) {
        await this.rtc.setRemoteDescription(answer);
    }

    async sendMessage(data) {
        if (!this.rtc.channel)
            return;

        this.rtc.channel.send(data);
    }
}