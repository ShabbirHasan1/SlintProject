import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
async function borrarPago(pos, e) {
    return await invoke("eliminar_pago", { "pos": pos, "index": e.currentTarget.parentElement.id });
}

function Pago({ pagado, medios_pago, monto, index,  borrar, agregar }) {
    const boton = pagado ? <input value="Borrar" onClick={borrar} type="button" id="boton-borrar-pago"></input> : <input value="Cash" type="submit" id="boton-agregar-pago"></input>
    const [seleccionado, setSeleccionado] = useState(medios_pago[0]);
    const [montoAct, setMontoAct] = useState(""+monto);
    
    
    const input = pagado ? <input type="number" placeholder={montoAct} readOnly={pagado} disabled="disabled" className="input-monto"  step="0.01" /> : 
    <input type="number" value={montoAct}  onChange={(e)=>{
        setMontoAct(e.currentTarget.value)}} className="input-monto" id="input-activo" step="0.01" />
    
    
    const opts = medios_pago.map(function (opt, i) {
        let sel;
        if (i == 0) {
            sel = "selected";
        } else {
            sel = "";
        }
        return <option key={i} id={i} defaultValue={sel} value={opt}>{opt}</option>
    });
    
    return (<form className="pago" id={index} onSubmit={(e) => agregar(e, seleccionado, montoAct)}>
        {input}
        <select  onChange={(e) => { setSeleccionado(e.currentTarget.value) }} className="opciones-pagos">
            {opts}
        </select>
        {boton}
    </form>)
}

export default Pago;