<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Employee_Contact_Detail</name>
   <tag></tag>
   <elementGuidId>71d7a461-27f3-4b34-8d62-7d11043b9278</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/api/v1/employee/19/contact-detail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>7e3fe87e-5291-41fc-b3dc-4858b9f1e009</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>1b17959b-e613-4158-8db8-1072d23303bf</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data.id', &quot;19&quot;)
WS.verifyElementPropertyValue(response, 'data.code', &quot;0012&quot;)
WS.verifyElementPropertyValue(response, 'data.fullName', &quot;Sakura Kinomoto&quot;)
WS.verifyElementPropertyValue(response, 'data.addressStreet1', &quot;Jl. Bunga Mawar Putih no.8&quot;)
WS.verifyElementPropertyValue(response, 'data.addressStreet2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'data.city', &quot;Jakarta Selatan&quot;)
WS.verifyElementPropertyValue(response, 'data.state', &quot;DKI Jakarta&quot;)
WS.verifyElementPropertyValue(response, 'data.zip', &quot;23456&quot;)
WS.verifyElementPropertyValue(response, 'data.county', &quot;INDONESIA&quot;)
WS.verifyElementPropertyValue(response, 'data.homeTelephone', &quot;0218008932&quot;)
WS.verifyElementPropertyValue(response, 'data.mobile', &quot;08123456789&quot;)
WS.verifyElementPropertyValue(response, 'data.otherEmail', &quot;sakura.kinomoto.edit@email.com&quot;)
WS.verifyElementPropertyValue(response, 'data.workEmail', &quot;kantor@email.com&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
