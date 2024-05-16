import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";
import Pago from "./Pago";

async function borrar_pago(pos, e) {
  console.log(e.currentTarget.parentElement.id)
  return await invoke("eliminar_pago", { "pos": pos, "id": e.currentTarget.parentElement.id });
}

async function agregar_pago(medio_pago, monto, pos) {
  return await invoke("agregar_pago", { "medioPago": medio_pago, "monto": monto, "pos": pos });
}

function Pagos({ pagos, medios_pago, monto, pos, isProd, prodFoc,credito,setDisabledCli }) {
  const [pagosVec, setPagosVec] = useState(mapearPagos(pagos))
  const [focused, setFocused] = useState(prodFoc?"not-focused":"");
  const [cred,setCred] = useState(credito);
  const [rend, setRent] = useState(<>
    <article id="pagos" className={"focuseable " + focused}  >
      {pagosVec}
      <Pago pagado={false} isProd={isProd} credito={cred} id={0} medios_pago={medios_pago} monto={monto} pos={pos} borrar={(e) => { console.log(e); borrar_pago(pos, e, ) }} agregar={cash} />
    </article>
    <p>Resta pagar: {monto}</p>
  </>)
  useEffect(()=>{setCred(credito)},[credito]);
  useEffect(()=>{setPagosVec(mapearPagos(pagos))},[pagos])
  useEffect(() => {setFocused(prodFoc?"not-focused":"")}, [prodFoc])
  useEffect(()=>{
    if(pagosVec.length==0){
      setDisabledCli("");
    }else{
      setDisabledCli("disabled");
    }
    setRent(<>
    <article id="pagos" className={"focuseable " + focused}  >
      {pagosVec}
      <Pago pagado={false} isProd={isProd} credito={cred} id={0} medios_pago={medios_pago} monto={monto} pos={pos} borrar={(e) => { console.log(e); borrar_pago(pos, e, ) }} agregar={cash} />
    </article>
    <p>Resta pagar: {monto}</p>
  </>)},[pagosVec,focused,cred])
  function click(e){
    console.log(e.currentTarget);
    document.getElementById("input-activo").select();
  }
  function mapearPagos(pagos) {
    return pagos.map(function (pago, i) {
      return <Pago key={i} isProd={isProd} pagado={true} medios_pago={[pago.medio_pago.medio]} monto={pago.monto} id={pago.int_id} borrar={(e) => borrar_pago(pos, e).then(pagos=>setPagosVec(mapearPagos(pagos)))} agregar={cash} />
    })
  }
  function cash(e, seleccionado, montoAct) {
    e.preventDefault();
    console.log("cash")
    agregar_pago(seleccionado, montoAct, pos).then(pagos => {console.log(pagos);setPagosVec(mapearPagos(pagos))});
  }



  return (rend)
 
  
}


export default Pagos;