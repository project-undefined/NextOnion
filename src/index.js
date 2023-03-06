import { MqttClient } from "@iota/mqtt.js";
import { SingleNodeClient, sendData } from "@iota/iota.js";

const mqttClient = new MqttClient("https://api.lb-0.h.chrysalis-devnet.iota.cafe");
const client     = new SingleNodeClient("https://api.lb-0.h.chrysalis-devnet.iota.cafe");

/*mqttClient.messages((topic, data, raw) => {
  //console.log(topic, data); 
}); */

console.log(client);
sendData(client, "h", "k")
  .then((d)=>console.log(d));